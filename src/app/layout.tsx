import { Inter } from "next/font/google";
import "./globals.css";
import QueryClient from "@/components/QueryClientProvider";

const inter = Inter({ subsets: ["latin"] });

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <QueryClient>
      <html lang="en">
        <body className={inter.className}>{children}</body>
      </html>
    </QueryClient>
  );
}
