import { ArrowRight, Github, FileCode, Cpu, Zap } from 'lucide-react';
import Link from 'next/link';
import styles from './index.module.css';

export default function Home() {
  return (
    <div className={styles.minHeightScreen}>
      <main className={`${styles.container} ${styles.textCenter}`}>
        <header className={styles.header}>
          <h1 className={styles.title}>Welcome to Rappy</h1>
          <p className={styles.subtitle}>Your Web3 Project Launchpad</p>
        </header>

        <section className={styles.section}>
          <h2 className={styles.readyToBuild}>Features</h2>
          <div className={`${styles.featureGrid} ${styles.featureGridMd}`}>
            <FeatureCard
              icon={<FileCode className={styles.cardIcon} />}
              title="Next.js Frontend"
              description="Modern React framework for building fast, SEO-friendly web applications"
            />
            <FeatureCard
              icon={<Cpu className={styles.cardIcon} />}
              title="Smart Contract Development"
              description="Choose between Foundry or Hardhat for your Ethereum development needs"
            />
            <FeatureCard
              icon={<Zap className={styles.cardIcon} />}
              title="Developer Experience"
              description="Includes pre-configured testing, formatting, and development scripts"
            />
          </div>
        </section>

        <section className={styles.textCenter}>
          <h2 className={styles.readyToBuild}>Ready to Build?</h2>
          <div className={styles.buttonGroup}>
            <Link
              href="https://github.com/devrapture/rappy"
              target="_blank"
              className={`${styles.button} ${styles.buttonBlue}`}
            >
              <Github className="mr-2" />
              GitHub
            </Link>
            <Link
              href="/docs"
              target="_blank"
              className={`${styles.button} ${styles.buttonGray}`}
            >
              <ArrowRight className="mr-2" />
              Documentation
            </Link>
          </div>
        </section>
      </main>

      <footer className={styles.footer}>
        <p>&copy; {new Date().getFullYear()} Rappy. All rights reserved.</p>
      </footer>
    </div>
  );
}

function FeatureCard({
  icon,
  title,
  description,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
}) {
  return (
    <div className={styles.card}>
      <div className={styles.cardIcon}>{icon}</div>
      <h3 className={styles.cardTitle}>{title}</h3>
      <p className={styles.cardDescription}>{description}</p>
    </div>
  );
}