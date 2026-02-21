import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "FHIR Builder",
  description: "Kenya-to-FHIR Bridge - Healthcare Interoperability Tool",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body suppressHydrationWarning>{children}</body>
    </html>
  );
}
