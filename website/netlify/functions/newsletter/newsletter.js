const { MongoClient } = require('mongodb');
const validator = require('validator');
const crypto = require('crypto');

// Environment variables for production
const MONGODB_URI = process.env.MONGODB_URI || 'mongodb://localhost:27017';
const DATABASE_NAME = process.env.DATABASE_NAME || 'neo_rust_website';
const COLLECTION_NAME = 'newsletter_subscribers';
const MAILCHIMP_API_KEY = process.env.MAILCHIMP_API_KEY;
const MAILCHIMP_LIST_ID = process.env.MAILCHIMP_LIST_ID;

// Rate limiting storage (in production, use Redis)
const rateLimitStore = new Map();

exports.handler = async (event, context) => {
  // Set CORS headers
  const headers = {
    'Access-Control-Allow-Origin': '*',
    'Access-Control-Allow-Headers': 'Content-Type',
    'Access-Control-Allow-Methods': 'POST, OPTIONS',
    'Content-Type': 'application/json'
  };

  // Handle preflight requests
  if (event.httpMethod === 'OPTIONS') {
    return {
      statusCode: 200,
      headers,
      body: ''
    };
  }

  // Only allow POST requests
  if (event.httpMethod !== 'POST') {
    return {
      statusCode: 405,
      headers,
      body: JSON.stringify({
        error: 'Method not allowed',
        message: 'Only POST requests are allowed'
      })
    };
  }

  try {
    // Parse request body
    let body;
    try {
      body = JSON.parse(event.body);
    } catch (error) {
      return {
        statusCode: 400,
        headers,
        body: JSON.stringify({
          error: 'Invalid JSON',
          message: 'Request body must be valid JSON'
        })
      };
    }

    const { email, name, interests = [] } = body;

    // Validate email
    if (!email || !validator.isEmail(email)) {
      return {
        statusCode: 400,
        headers,
        body: JSON.stringify({
          error: 'Invalid email',
          message: 'Please provide a valid email address'
        })
      };
    }

    // Validate name (optional but if provided, should be reasonable)
    if (name && (name.length < 1 || name.length > 100)) {
      return {
        statusCode: 400,
        headers,
        body: JSON.stringify({
          error: 'Invalid name',
          message: 'Name must be between 1 and 100 characters'
        })
      };
    }

    // Rate limiting check
    const clientIP = event.headers['x-forwarded-for'] || event.headers['x-real-ip'] || 'unknown';
    const rateLimitKey = `newsletter_${clientIP}`;
    const now = Date.now();
    const rateLimit = rateLimitStore.get(rateLimitKey);

    if (rateLimit && now - rateLimit.lastRequest < 60000) { // 1 minute cooldown
      if (rateLimit.attempts >= 3) {
        return {
          statusCode: 429,
          headers,
          body: JSON.stringify({
            error: 'Rate limit exceeded',
            message: 'Too many subscription attempts. Please try again later.'
          })
        };
      }
      rateLimit.attempts++;
    } else {
      rateLimitStore.set(rateLimitKey, { lastRequest: now, attempts: 1 });
    }

    // Connect to MongoDB
    const client = new MongoClient(MONGODB_URI);
    await client.connect();
    
    const db = client.db(DATABASE_NAME);
    const collection = db.collection(COLLECTION_NAME);

    // Check if email already exists
    const existingSubscriber = await collection.findOne({ email: email.toLowerCase() });
    
    if (existingSubscriber) {
      await client.close();
      return {
        statusCode: 409,
        headers,
        body: JSON.stringify({
          error: 'Already subscribed',
          message: 'This email address is already subscribed to our newsletter'
        })
      };
    }

    // Create subscriber record
    const subscriber = {
      email: email.toLowerCase(),
      name: name || null,
      interests: Array.isArray(interests) ? interests.slice(0, 10) : [], // Limit interests
      subscribedAt: new Date(),
      confirmed: false,
      confirmationToken: crypto.randomBytes(32).toString('hex'),
      source: 'website',
      ipAddress: clientIP,
      userAgent: event.headers['user-agent'] || 'unknown'
    };

    // Insert subscriber
    const result = await collection.insertOne(subscriber);
    
    // Add to Mailchimp if configured
    if (MAILCHIMP_API_KEY && MAILCHIMP_LIST_ID) {
      try {
        await addToMailchimp(email, name, interests);
      } catch (mailchimpError) {
        console.error('Mailchimp error:', mailchimpError);
        // Don't fail the request if Mailchimp fails
      }
    }

    // Send confirmation email (in production, use a proper email service)
    try {
      await sendConfirmationEmail(email, name, subscriber.confirmationToken);
    } catch (emailError) {
      console.error('Email sending error:', emailError);
      // Don't fail the request if email sending fails
    }

    await client.close();

    // Clean up rate limit store periodically
    if (rateLimitStore.size > 1000) {
      const cutoff = now - 3600000; // 1 hour ago
      for (const [key, value] of rateLimitStore.entries()) {
        if (value.lastRequest < cutoff) {
          rateLimitStore.delete(key);
        }
      }
    }

    return {
      statusCode: 201,
      headers,
      body: JSON.stringify({
        success: true,
        message: 'Successfully subscribed! Please check your email to confirm your subscription.',
        data: {
          email: email.toLowerCase(),
          subscribedAt: subscriber.subscribedAt,
          needsConfirmation: true
        }
      })
    };

  } catch (error) {
    console.error('Newsletter subscription error:', error);
    
    return {
      statusCode: 500,
      headers,
      body: JSON.stringify({
        error: 'Internal server error',
        message: 'An error occurred while processing your subscription. Please try again later.'
      })
    };
  }
};

// Add subscriber to Mailchimp
async function addToMailchimp(email, name, interests) {
  if (!MAILCHIMP_API_KEY || !MAILCHIMP_LIST_ID) {
    throw new Error('Mailchimp not configured');
  }

  const fetch = require('node-fetch');
  const datacenter = MAILCHIMP_API_KEY.split('-')[1];
  const url = `https://${datacenter}.api.mailchimp.com/3.0/lists/${MAILCHIMP_LIST_ID}/members`;

  const data = {
    email_address: email,
    status: 'pending', // Requires confirmation
    merge_fields: {
      FNAME: name || '',
      INTERESTS: interests.join(', ')
    },
    tags: ['website-signup']
  };

  const response = await fetch(url, {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${MAILCHIMP_API_KEY}`,
      'Content-Type': 'application/json'
    },
    body: JSON.stringify(data)
  });

  if (!response.ok) {
    const error = await response.text();
    throw new Error(`Mailchimp API error: ${error}`);
  }

  return await response.json();
}

// Send confirmation email (placeholder - use a real email service in production)
async function sendConfirmationEmail(email, name, token) {
  // In production, integrate with services like:
  // - SendGrid
  // - AWS SES
  // - Mailgun
  // - Postmark
  
  const confirmationUrl = `${process.env.SITE_URL || 'https://neo-rust.com'}/confirm-subscription?token=${token}`;
  
  console.log(`Confirmation email would be sent to ${email}`);
  console.log(`Confirmation URL: ${confirmationUrl}`);
  
  // Example with SendGrid (uncomment and configure in production):
  /*
  const sgMail = require('@sendgrid/mail');
  sgMail.setApiKey(process.env.SENDGRID_API_KEY);
  
  const msg = {
    to: email,
    from: 'noreply@neo-rust.com',
    subject: 'Confirm your Neo Rust SDK newsletter subscription',
    html: `
      <h2>Welcome to the Neo Rust SDK Newsletter!</h2>
      <p>Hi ${name || 'there'},</p>
      <p>Thank you for subscribing to our newsletter. Please confirm your subscription by clicking the link below:</p>
      <p><a href="${confirmationUrl}" style="background-color: #007bff; color: white; padding: 10px 20px; text-decoration: none; border-radius: 5px;">Confirm Subscription</a></p>
      <p>If you didn't subscribe to this newsletter, you can safely ignore this email.</p>
      <p>Best regards,<br>The Neo Rust SDK Team</p>
    `
  };
  
  await sgMail.send(msg);
  */
  
  return true;
}