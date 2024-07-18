import { Inter, Newsreader } from "next/font/google";
import "./globals.css";
import QueryClient from "@/components/QueryClientProvider";
import Header from "@/components/Header";
import classNames from "classnames";
import Footer from "@/components/Footer";

const inter = Inter({ subsets: ["latin"], variable: "--font-inter" });

const newsreader = Newsreader({
  subsets: ["latin"],
  variable: "--font-newsreader",
});

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <QueryClient>
      <html
        lang="en"
        className={classNames(inter.variable, newsreader.variable)}
      >
        <body className="min-h-screen flex flex-col">
          <Header />
          {children}
          <Footer />
        </body>
      </html>
    </QueryClient>
  );
}
