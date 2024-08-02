import { Inter, Newsreader } from "next/font/google";
import "./globals.css";
import QueryClient from "@/components/QueryClientProvider";
import Header from "@/components/Header";
import classNames from "classnames";
import Footer from "@/components/Footer";
import { AuthProvider } from "@/components/AuthContext";

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
      <AuthProvider>
        <html
          lang="en"
          className={classNames(inter.variable, newsreader.variable)}
        >
          <body className="flex min-h-screen flex-col">
            <Header />
            {children}
            <Footer />
          </body>
        </html>
      </AuthProvider>
    </QueryClient>
  );
}
