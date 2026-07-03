import type { Metadata } from "next";
import { Bebas_Neue, Share_Tech_Mono } from "next/font/google";
import "./globals.css";

const bebasNeue = Bebas_Neue({ 
  weight: "400", 
  subsets: ["latin"],
  variable: "--font-bebas",
});

const shareTechMono = Share_Tech_Mono({ 
  weight: "400", 
  subsets: ["latin"],
  variable: "--font-share-tech",
});

export const metadata: Metadata = {
  title: "YURA JIG SYSTEM",
  description: "Jig Management MVP",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="ko">
      <body className={`${bebasNeue.variable} ${shareTechMono.variable} font-mono bg-slate-900 text-slate-200 min-h-screen`}>
        {children}
      </body>
    </html>
  );
}