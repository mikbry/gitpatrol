import React, { useState, useEffect } from 'react';
import { FileSearch, GitBranch, ShieldPlus, Upload, Sun, Moon, Laptop, Mail } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import ScanDialog from './ScanDialog';

type Theme = 'light' | 'dark' | 'system';

export default function App() {
  const [report, setReport] = useState<string>('');
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [isContactDialogOpen, setIsContactDialogOpen] = useState(false);
  const [theme, setTheme] = useState<Theme>('system');
  const [email, setEmail] = useState('');

  useEffect(() => {
    const root = window.document.documentElement;
    root.classList.remove('light', 'dark');
    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';
      root.classList.add(systemTheme);
    } else {
      root.classList.add(theme);
    }
  }, [theme]);

  const handleEmailSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    // Here you would typically send the email to your backend
    console.log('Email submitted:', email);
    setIsContactDialogOpen(false);
    setEmail('');
  };

  const handleScanStart = async (text: string | undefined) => {
    if (!text) return;

    try {
      const { WasmScanner } = await import('@/lib/wasm');
      const scanner = new WasmScanner();
      scanner.set_content(text);
      const hasSuspiciousCode = await scanner.scan();

      setReport(
        hasSuspiciousCode ? '🚨 Suspicious patterns detected!' : '✅ No suspicious patterns found',
      );
    } catch (error) {
      console.error('Scan error:', error);
      setReport('❌ Error scanning file');
    }
  };

  return (
    <div className="flex min-h-screen flex-col">
      <header className="fixed left-0 right-0 top-0 z-50 flex h-14 items-center justify-between bg-orange-600/80 px-4 text-white backdrop-blur-md lg:px-6">
        <a className="flex items-center justify-center" href="/">
          <ShieldPlus className="h-6 w-6 text-white" aria-hidden="true" />
          <span className="ml-2 text-2xl font-bold">GitPatrol</span>
        </a>
        <nav className="flex items-center gap-4 sm:gap-6">
          <a className="text-sm font-medium underline-offset-4 hover:underline" href="#features">
            Features
          </a>
          <a className="text-sm font-medium underline-offset-4 hover:underline" href="#why-scan">
            Why Scan?
          </a>
          <a
            className="text-sm font-medium underline-offset-4 hover:underline"
            href="#contact"
            onClick={() => setIsContactDialogOpen(true)}
          >
            Contact
          </a>
          <a
            className="text-sm font-medium underline-offset-4 hover:underline"
            href="https://github.com/mikbry/gitpatrol"
            target="_blank"
            rel="noopener noreferrer"
          >
            GitHub
          </a>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button
                variant="outline"
                size="icon"
                className="border-white bg-white text-orange-500 hover:bg-orange-100"
              >
                <Sun className="h-[1.2rem] w-[1.2rem] rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
                <Moon className="absolute h-[1.2rem] w-[1.2rem] rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
                <span className="sr-only">Toggle theme</span>
              </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="end">
              <DropdownMenuItem onClick={() => setTheme('light')}>
                <Sun className="mr-2 h-4 w-4" />
                <span>Light</span>
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setTheme('dark')}>
                <Moon className="mr-2 h-4 w-4" />
                <span>Dark</span>
              </DropdownMenuItem>
              <DropdownMenuItem onClick={() => setTheme('system')}>
                <Laptop className="mr-2 h-4 w-4" />
                <span>System</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </nav>
      </header>
      <main className="flex-1 pt-14">
        <section className="relative w-full overflow-hidden bg-gradient-to-br from-orange-600 via-orange-500 to-orange-400 py-12 text-white md:py-24 lg:py-32 xl:py-48">
          <div className="container relative z-10 px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
                  Secure Your Code with GitPatrol
                </h1>
                <p className="mx-auto max-w-[700px] text-orange-100 md:text-xl">
                  Upload your JS files or ZIP archives and let GitPatrol scan for malware and scams.
                  Protect your projects from hidden threats.
                </p>
              </div>
              <div className="space-x-4">
                <Button
                  className="bg-white text-orange-500 hover:bg-orange-100"
                  onClick={() => setIsDialogOpen(true)}
                >
                  <ShieldPlus className="ml-2 h-4 w-4" aria-hidden="true" />
                  Scan now
                </Button>
                {/* <Button
                  variant="outline"
                  className="bg-transparent border-white text-white hover:bg-white hover:text-orange-500"
                >
                  Learn More
                </Button> */}
              </div>
            </div>
          </div>
        </section>
        <section
          id="why-scan"
          className="w-full bg-gray-100 py-12 md:py-24 lg:py-32 dark:bg-gray-800"
        >
          <div className="container px-4 md:px-6">
            <h2 className="mb-8 text-center text-3xl font-bold tracking-tighter text-orange-500 sm:text-4xl md:text-5xl dark:text-orange-400">
              Why Scan Unknown Repositories?
            </h2>
            <div className="grid items-stretch gap-6 lg:grid-cols-3 lg:gap-12">
              <Card className="flex flex-col border-orange-200 dark:border-orange-800">
                <CardHeader>
                  <CardTitle className="text-orange-500 dark:text-orange-400">
                    Hidden Malware
                  </CardTitle>
                </CardHeader>
                <CardContent className="flex-grow">
                  Malicious actors can hide harmful code within seemingly innocent repositories.
                  Regular scanning helps detect and prevent potential threats.
                </CardContent>
              </Card>
              <Card className="flex flex-col border-orange-200 dark:border-orange-800">
                <CardHeader>
                  <CardTitle className="text-orange-500 dark:text-orange-400">
                    Data Protection
                  </CardTitle>
                </CardHeader>
                <CardContent className="flex-grow">
                  Unverified code may contain scripts that compromise your data or system integrity.
                  Scanning ensures your information remains secure.
                </CardContent>
              </Card>
              <Card className="flex flex-col border-orange-200 dark:border-orange-800">
                <CardHeader>
                  <CardTitle className="text-orange-500 dark:text-orange-400">
                    Trust Verification
                  </CardTitle>
                </CardHeader>
                <CardContent className="flex-grow">
                  Even repositories from trusted sources can be compromised. Scanning provides an
                  extra layer of verification and peace of mind.
                </CardContent>
              </Card>
            </div>
          </div>
        </section>
        <section id="features" className="w-full py-12 md:py-24 lg:py-32">
          <div className="container px-4 md:px-6">
            <h2 className="mb-8 text-center text-3xl font-bold tracking-tighter text-orange-500 sm:text-4xl md:text-5xl dark:text-orange-400">
              GitPatrol Features
            </h2>
            <div className="grid items-stretch gap-6 lg:grid-cols-3 lg:gap-12">
              <Card className="flex flex-col border-orange-200 dark:border-orange-800">
                <CardHeader>
                  <Upload
                    className="mb-2 h-10 w-10 text-orange-500 dark:text-orange-400"
                    aria-hidden="true"
                  />
                  <CardTitle className="text-orange-500 dark:text-orange-400">
                    Easy Upload
                  </CardTitle>
                </CardHeader>
                <CardContent className="flex-grow">
                  Simply drag and drop your JS files or ZIP archives for instant scanning.
                </CardContent>
              </Card>
              <Card className="flex flex-col border-orange-200 dark:border-orange-800">
                <CardHeader>
                  <FileSearch
                    className="mb-2 h-10 w-10 text-orange-500 dark:text-orange-400"
                    aria-hidden="true"
                  />
                  <CardTitle className="text-orange-500 dark:text-orange-400">Deep Scan</CardTitle>
                </CardHeader>
                <CardContent className="flex-grow">
                  Our advanced algorithms thoroughly analyze your code for any potential threats or
                  malicious patterns.
                </CardContent>
              </Card>
              <Card className="flex flex-col border-orange-200 dark:border-orange-800">
                <CardHeader>
                  <GitBranch
                    className="mb-2 h-10 w-10 text-orange-500 dark:text-orange-400"
                    aria-hidden="true"
                  />
                  <CardTitle className="text-orange-500 dark:text-orange-400">
                    Open Source & Git Integration
                  </CardTitle>
                </CardHeader>
                <CardContent className="flex-grow">
                  Fully open source and seamlessly integrate GitPatrol with your favorite Git
                  providers for continuous protection. Join our community on GitHub to contribute
                  and help make code safer for everyone.
                </CardContent>
              </Card>
            </div>
          </div>
        </section>
        <section className="w-full bg-gradient-to-br from-orange-600 via-orange-500 to-orange-400 py-12 text-white md:py-24 lg:py-32">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
                  Ready to Secure Your Code?
                </h2>
                <p className="mx-auto max-w-[600px] text-orange-100 md:text-xl">
                  Start scanning your repositories today and protect your projects from hidden
                  threats.
                </p>
              </div>
              <Button
                className="bg-white text-orange-500 hover:bg-orange-100"
                onClick={() => setIsDialogOpen(true)}
              >
                <ShieldPlus className="ml-2 h-4 w-4" aria-hidden="true" />
                Scan Now
              </Button>
            </div>
          </div>
        </section>
      </main>
      <footer className="flex w-full shrink-0 flex-col items-center gap-2 border-t border-orange-200 px-4 py-6 sm:flex-row md:px-6 dark:border-orange-800">
        <p className="text-xs text-gray-500 dark:text-gray-400">
          © 2024 GitPatrol. All rights reserved.
        </p>
        <nav className="flex gap-4 sm:ml-auto sm:gap-6">
          <a
            className="text-xs text-orange-500 underline-offset-4 hover:underline dark:text-orange-400"
            href="/terms"
          >
            Terms of Service
          </a>
          <a
            className="text-xs text-orange-500 underline-offset-4 hover:underline dark:text-orange-400"
            href="/privacy"
          >
            Privacy
          </a>
        </nav>
      </footer>
      <ScanDialog
        isOpen={isDialogOpen}
        onOpenChange={setIsDialogOpen}
        report={report}
        onScan={handleScanStart}
      />
      <Dialog open={isContactDialogOpen} onOpenChange={setIsContactDialogOpen}>
        <DialogContent className="sm:max-w-[425px]">
          <DialogHeader>
            <DialogTitle className="flex items-center gap-2 text-2xl font-bold text-orange-500 dark:text-orange-400">
              <Mail className="h-6 w-6" />
              Contact Us
            </DialogTitle>
          </DialogHeader>
          <form onSubmit={handleEmailSubmit}>
            <div className="grid gap-4 py-4">
              <div className="grid grid-cols-4 items-center gap-4">
                <Label htmlFor="email" className="text-right">
                  Email
                </Label>
                <Input
                  id="email"
                  type="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  className="col-span-3"
                  required
                />
              </div>
            </div>
            <DialogFooter>
              <Button type="submit" className="bg-orange-500 text-white hover:bg-orange-600">
                Submit
              </Button>
            </DialogFooter>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
}
