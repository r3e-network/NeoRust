import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: '🎨 Beautiful Desktop GUI',
    Svg: require('@site/static/img/gui-feature.svg').default,
    description: (
      <>
        Modern, responsive interface built with Tauri and React. 
        Manage wallets, trade NFTs, and interact with the blockchain 
        through an intuitive visual interface.
      </>
    ),
  },
  {
    title: '💻 Powerful CLI Tools',
    Svg: require('@site/static/img/cli-feature.svg').default,
    description: (
      <>
        Professional command-line interface with beautiful colored output, 
        progress indicators, and comprehensive blockchain operations. 
        Perfect for automation and CI/CD pipelines.
      </>
    ),
  },
  {
    title: '📚 Production-Ready SDK',
    Svg: require('@site/static/img/sdk-feature.svg').default,
    description: (
      <>
        Comprehensive Rust library with zero panics, full test coverage, 
        and enterprise-grade reliability. Build robust Neo N3 applications 
        with confidence.
      </>
    ),
  },
  {
    title: '🔒 Enterprise Security',
    Svg: require('@site/static/img/security-feature.svg').default,
    description: (
      <>
        Hardware wallet support, encrypted storage, and secure key management. 
        Built with Rust's memory safety and comprehensive security audits.
      </>
    ),
  },
  {
    title: '🚀 High Performance',
    Svg: require('@site/static/img/performance-feature.svg').default,
    description: (
      <>
        Optimized for high-throughput applications with async/await support, 
        efficient memory usage, and minimal overhead. Scale to enterprise needs.
      </>
    ),
  },
  {
    title: '🌐 Multi-Network Support',
    Svg: require('@site/static/img/network-feature.svg').default,
    description: (
      <>
        Connect to MainNet, TestNet, and private networks. 
        Automatic health monitoring, failover support, and 
        real-time blockchain synchronization.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <div className={styles.featureSvg}>
          <div className={styles.featureIcon}>
            {title.split(' ')[0]}
          </div>
        </div>
      </div>
      <div className="text--center padding-horiz--md">
        <h3 className={styles.featureTitle}>{title}</h3>
        <p className={styles.featureDescription}>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          <div className="col col--12">
            <h2 className={styles.featuresTitle}>
              Why Choose NeoRust?
            </h2>
            <p className={styles.featuresSubtitle}>
              Built by developers, for developers. NeoRust provides everything you need 
              to build production-ready Neo N3 applications.
            </p>
          </div>
        </div>
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
} 