import Head from '../components/Head'
import '../styles/global.css'
import type { AppProps } from 'next/app'

export default function App({ Component, pageProps }: AppProps) {

  return (
    <>  
      <Head />
      <Component {...pageProps} />
    </>
  )
}
