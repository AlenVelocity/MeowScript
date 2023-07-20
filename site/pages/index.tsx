import Code from '../components/Code'
import Features from '../components/Features'
import Head from '../components/Head'
import { GetStaticProps } from 'next'
import React from 'react'
import ReactMarkdown from 'react-markdown'

import { readDocsFile } from '../lib/docs'

type Props = {
  docs: string
}

const Landing = ({ docs }: Props) => {
  return (
    <>
      <div className="bg-bg font-light relative">
        <div className="relative pt-6 pb-16 sm:pb-24">
          <main className="mt-16 mx-auto max-w-7xl px-4 sm:mt-24">
            <div className="text-center">
              <h1 className="text-4xl tracking-tight  text-gray-50 sm:text-5xl md:text-6xl">
                MeowScript🐾
              </h1>
              <p className="mt-3 max-w-md mx-auto text-base text-gray-300 sm:text-lg md:mt-5 md:text-xl md:max-w-3xl">
                The Purrfect Programming Language
              </p>
              <div className="mt-5 max-w-md mx-auto sm:flex sm:justify-center md:mt-8">
                <div className="rounded-md shadow">
                  <a
                    href="https://github.com/AlenVelocity/MeowScript/"
                    className="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-white bg-orange-600 hover:bg-yellow-700 md:py-4 md:text-lg md:px-10">
                    View source
                  </a>
                </div>
                <div className="mt-3 rounded-md shadow sm:mt-0 sm:ml-3">
                  <a
                    href="#code"
                    className="w-full flex items-center justify-center px-8 py-3 border border-transparent text-base font-medium rounded-md text-orange-600 bg-white hover:bg-gray-50 md:py-4 md:text-lg md:px-10">
                    Playground
                  </a>
                </div>
              </div>
              <div className="mt-4">
                <h1 className="text-gray-50">
                  Made by{' '}
                  <a
                    className="text-yellow-500 hover:text-yellow-600"
                    href="https://twitter.com/Alenvelocity">
                    @Alen
                  </a>
                  , inspired by{' '}
                  <a
                    href="https://www.google.com/search?q=cats"
                    className="text-yellow-400 hover:text-gray-100">
                    🐱
                  </a>
                </h1>
              </div>
            </div>
          </main>
        </div>
        <Features />
        <Code />
        <div className="docs mt-12 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="max-w-3xl mx-auto overflow-auto">
            <ReactMarkdown>{docs}</ReactMarkdown>
          </div>
        </div>
      </div>
    </>
  )
}

export default Landing

export const getStaticProps: GetStaticProps = async () => {
  const docs = await readDocsFile()
  return {
    props: {
      docs
    }
  }
}
