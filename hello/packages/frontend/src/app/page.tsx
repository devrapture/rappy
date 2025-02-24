import { ArrowRight, Github, FileCode, Cpu, Zap } from 'lucide-react'
import Link from 'next/link'

export default function Home() {
  return (
    <div className="min-h-screen bg-gradient-to-b from-gray-900 to-gray-800 text-white">
      <main className="container mx-auto px-4 py-16">
        <header className="text-center mb-16">
          <h1 className="text-6xl font-bold mb-4">Welcome to Rappy</h1>
          <p className="text-xl text-gray-300">Your Web3 Project Launchpad</p>
        </header>


        <section className="mb-16">
          <h2 className="text-3xl font-semibold mb-6">Features</h2>
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            <FeatureCard 
              icon={<FileCode className="w-8 h-8" />}
              title="Next.js Frontend"
              description="Modern React framework for building fast, SEO-friendly web applications"
            />
            <FeatureCard 
              icon={<Cpu className="w-8 h-8" />}
              title="Smart Contract Development"
              description="Choose between Foundry or Hardhat for your Ethereum development needs"
            />
            <FeatureCard 
              icon={<Zap className="w-8 h-8" />}
              title="Developer Experience"
              description="Includes pre-configured testing, formatting, and development scripts"
            />
          </div>
        </section>

        <section className="text-center">
          <h2 className="text-3xl font-semibold mb-6">Ready to Build?</h2>
          <div className="space-x-4">
            <Link href="https://github.com/devrapture/rappy" target='_blank' className="inline-flex items-center bg-blue-600 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded transition duration-300">
              <Github className="w-5 h-5 mr-2" />
              GitHub
            </Link>
            <Link href="/docs" target='_blank' className="inline-flex items-center bg-gray-700 hover:bg-gray-600 text-white font-bold py-2 px-4 rounded transition duration-300">
              <ArrowRight className="w-5 h-5 mr-2" />
              Documentation
            </Link>
          </div>
        </section>
      </main>

      <footer className="text-center py-8 text-gray-400">
        <p>&copy; {new Date().getFullYear()} Rappy. All rights reserved.</p>
      </footer>
    </div>
  )
}

function FeatureCard({ icon, title, description }: { icon: React.ReactNode, title: string, description: string }) {
  return (
    <div className="bg-gray-800 rounded-lg p-6 shadow-lg">
      <div className="text-blue-400 mb-4">{icon}</div>
      <h3 className="text-xl font-semibold mb-2">{title}</h3>
      <p className="text-gray-300">{description}</p>
    </div>
  )
}

