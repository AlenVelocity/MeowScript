import type { Metadata, Viewport } from "next";
import { Baloo_2 } from "next/font/google";
import localFont from "next/font/local";
import "./globals.css";

const baloo = Baloo_2({
  subsets: ["latin"],
  weight: ["700", "800"],
  variable: "--font-baloo",
  display: "swap",
});

const fira = localFont({
  src: "../fonts/FiraCode-Regular.woff2",
  weight: "400",
  style: "normal",
  variable: "--font-fira",
  display: "swap",
});

const title = "MeowScript";
const description =
  "The purrfect programming language. A small scripting language with cat-pun keywords, interpreted in Rust and running in your browser.";

export const metadata: Metadata = {
  metadataBase: new URL("https://meowscript.vercel.app"),
  title: {
    default: title,
    template: `%s · ${title}`,
  },
  description,
  openGraph: {
    title,
    description,
    siteName: title,
    type: "website",
    images: ["/og.png"],
  },
  twitter: {
    card: "summary",
    title,
    description,
    creator: "@alenvelocity",
    images: ["/og.png"],
  },
};

export const viewport: Viewport = {
  themeColor: "#171310",
  colorScheme: "dark",
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html
      lang="en"
      data-scroll-behavior="smooth"
      className={`${baloo.variable} ${fira.variable} h-full antialiased`}
    >
      <body className="flex min-h-full flex-col">{children}</body>
    </html>
  );
}
