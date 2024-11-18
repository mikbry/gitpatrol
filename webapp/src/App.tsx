import React, { useState, useEffect } from 'react'
import { Button } from "@/components/ui/button"
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card"
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from "@/components/ui/dropdown-menu"
import { ArrowRight, FileSearch, GitBranch, Shield, Upload, Sun, Moon, Laptop } from 'lucide-react'

type Theme = 'light' | 'dark' | 'system'

export default function App() {
  const [file, setFile] = useState<File | null>(null);
  const [result, setResult] = useState<string>('');
  const [isDialogOpen, setIsDialogOpen] = useState(false)
  const [isScanning, setIsScanning] = useState(false)
  const [theme, setTheme] = useState<Theme>('system')

  useEffect(() => {
    const root = window.document.documentElement
    root.classList.remove('light', 'dark')
    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
      root.classList.add(systemTheme)
    } else {
      root.classList.add(theme)
    }
  }, [theme])

  /* const handleFileChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    if (event.target.files && event.target.files[0]) {
      setFile(event.target.files[0])
    }
  }

  const handleScanStart = () => {
    if (file) {
      setIsScanning(true)
      // Simulating a scan process
      setTimeout(() => {
        setIsScanning(false)
        setIsDialogOpen(false)
        alert(`Scan completed for ${file.name}`)
        setFile(null)
      }, 3000)
    }
  } */

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  const handleScanStart = async () => {
    if (!file) return;

    try {
      setIsScanning(true)
      const text = await file.text();
      const { WasmScanner } = await import('@/lib/wasm');
      const scanner = new WasmScanner();
      scanner.set_content(text);
      const hasSuspiciousCode = await scanner.scan();

      setResult(
        hasSuspiciousCode ? '🚨 Suspicious patterns detected!' : '✅ No suspicious patterns found',
      );
    } catch (error) {
      console.error('Scan error:', error);
      setResult('❌ Error scanning file');
    }
    setIsScanning(false)
  };
  return (
    <div className="flex flex-col min-h-screen">
      <header className="px-4 lg:px-6 h-14 flex items-center justify-between bg-orange-500 text-white">
        <a className="flex items-center justify-center" href="#">
          <Shield className="h-6 w-6 text-white" />
          <span className="ml-2 text-2xl font-bold">GitPatrol</span>
        </a>
        <nav className="flex items-center gap-4 sm:gap-6">
          <a className="text-sm font-medium hover:underline underline-offset-4" href="#features">
            Features
          </a>
          <a className="text-sm font-medium hover:underline underline-offset-4" href="#why-scan">
            Why Scan?
          </a>
          <a className="text-sm font-medium hover:underline underline-offset-4" href="#contact">
            Contact
          </a>
          <a 
            className="text-sm font-medium hover:underline underline-offset-4" 
            href="https://github.com/mikbry/gitpatrol"
            target="_blank"
            rel="noopener noreferrer"
          >
            GitHub
          </a>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <Button variant="outline" size="icon" className="bg-white text-orange-500 border-white hover:bg-orange-100">
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
      <main className="flex-1">
        <section className="w-full py-12 md:py-24 lg:py-32 xl:py-48 bg-orange-500 text-white">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <h1 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl lg:text-6xl/none">
                  Secure Your Code with GitPatrol
                </h1>
                <p className="mx-auto max-w-[700px] text-orange-100 md:text-xl">
                  Upload your JS files or ZIP archives and let GitPatrol scan for malware and scams. Protect your projects from hidden threats.
                </p>
              </div>
              <div className="space-x-4">
                <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
                  <DialogTrigger asChild>
                    <Button className="bg-white text-orange-500 hover:bg-orange-100">Get Started</Button>
                  </DialogTrigger>
                  <DialogContent className="bg-white">
                    <DialogHeader>
                      <DialogTitle className="text-orange-500">Upload File for Scanning</DialogTitle>
                    </DialogHeader>
                    <div className="grid gap-4 py-4">
                      <div className="grid grid-cols-4 items-center gap-4">
                        <Label htmlFor="file" className="text-right text-gray-700">
                          File
                        </Label>
                        <Input 
                          id="file" 
                          type="file" 
                          className="col-span-3 bg-gray-100 text-gray-900 border-orange-300 focus:border-orange-500 focus:ring-orange-500" 
                          onChange={handleFileChange} 
                          accept=".js,.zip" 
                        />
                      </div>
                    </div>
                    <Button 
                      onClick={handleScanStart} 
                      disabled={!file || isScanning}
                      className="bg-orange-500 text-white hover:bg-orange-600"
                    >
                      {isScanning ? 'Scanning...' : 'Start Scan'}
                    </Button>
                  </DialogContent>
                </Dialog>
                <Button variant="outline" className="bg-transparent border-white text-white hover:bg-white hover:text-orange-500">Learn More</Button>
              </div>
            </div>
          </div>
        </section>
        <section id="why-scan" className="w-full py-12 md:py-24 lg:py-32 bg-gray-100 dark:bg-gray-800">
          <div className="container px-4 md:px-6">
            <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl text-center mb-8 text-orange-500">
              Why Scan Unknown Repositories?
            </h2>
            <div className="grid gap-6 lg:grid-cols-3 lg:gap-12 items-start">
              <Card className="border-orange-200">
                <CardHeader>
                  <CardTitle className="text-orange-500">Hidden Malware</CardTitle>
                </CardHeader>
                <CardContent>
                  Malicious actors can hide harmful code within seemingly innocent repositories. Regular scanning helps detect and prevent potential threats.
                </CardContent>
              </Card>
              <Card className="border-orange-200">
                <CardHeader>
                  <CardTitle className="text-orange-500">Data Protection</CardTitle>
                </CardHeader>
                <CardContent>
                  Unverified code may contain scripts that compromise your data or system integrity. Scanning ensures your information remains secure.
                </CardContent>
              </Card>
              <Card className="border-orange-200">
                <CardHeader>
                  <CardTitle className="text-orange-500">Trust Verification</CardTitle>
                </CardHeader>
                <CardContent>
                  Even repositories from trusted sources can be compromised. Scanning provides an extra layer of verification and peace of mind.
                </CardContent>
              </Card>
            </div>
          </div>
        </section>
        <section id="features" className="w-full py-12 md:py-24 lg:py-32">
          <div className="container px-4 md:px-6">
            <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl text-center mb-8 text-orange-500">
              GitPatrol Features
            </h2>
            <div className="grid gap-6 lg:grid-cols-3 lg:gap-12 items-start">
              <Card className="border-orange-200">
                <CardHeader>
                  <Upload className="h-10 w-10 mb-2 text-orange-500" />
                  <CardTitle className="text-orange-500">Easy Upload</CardTitle>
                </CardHeader>
                <CardContent>
                  Simply drag and drop your JS files or ZIP archives for instant scanning.
                </CardContent>
              </Card>
              <Card className="border-orange-200">
                <CardHeader>
                  <FileSearch className="h-10 w-10 mb-2 text-orange-500" />
                  <CardTitle className="text-orange-500">Deep Scan</CardTitle>
                </CardHeader>
                <CardContent>
                  Our advanced algorithms thoroughly analyze your code for any potential threats or malicious patterns.
                </CardContent>
              </Card>
              <Card className="border-orange-200">
                <CardHeader>
                  <GitBranch className="h-10 w-10 mb-2 text-orange-500" />
                  <CardTitle className="text-orange-500">Open Source & Git Integration</CardTitle>
                </CardHeader>
                <CardContent>
                  Fully open source and seamlessly integrate GitPatrol with your favorite Git providers for continuous protection. Join our community on GitHub to contribute and help make code safer for everyone.
                </CardContent>
              </Card>
            </div>
          </div>
        </section>
        <section className="w-full py-12 md:py-24 lg:py-32 bg-orange-500 text-white">
          <div className="container px-4 md:px-6">
            <div className="flex flex-col items-center space-y-4 text-center">
              <div className="space-y-2">
                <h2 className="text-3xl font-bold tracking-tighter sm:text-4xl md:text-5xl">
                  Ready to Secure Your Code?
                </h2>
                <p className="mx-auto max-w-[600px] text-orange-100 md:text-xl">
                  Start scanning your repositories today and protect your projects from hidden threats.
                </p>
              </div>
              <Dialog open={isDialogOpen} onOpenChange={setIsDialogOpen}>
                <DialogTrigger asChild>
                  <Button className="bg-white text-orange-500 hover:bg-orange-100">
                    Get Started Now
                    <ArrowRight className="ml-2 h-4 w-4" />
                  </Button>
                </DialogTrigger>
                <DialogContent className="bg-white">
                  <DialogHeader>
                    <DialogTitle className="text-orange-500">Choose file for Scanning</DialogTitle>
                  </DialogHeader>
                  <div className="grid gap-4 py-4">
                    <div className="text-black">This a demo of GitPatrol, the scan is running on your machine, nothing is send to any server. It is a secure and sandboxed operation using WASM and Rust. If you think results are not ok, go to our Github page and add an issue. Thanks !</div>
                    <div className="grid grid-cols-4 items-center gap-4">
                      <Label htmlFor="file-upload" className="text-right text-gray-700">
                        File
                      </Label>
                      <Input id="file-upload" type="file" className="col-span-3 bg-gray-100 text-gray-900 border-orange-300 focus:border-orange-500 focus:ring-orange-500" onChange={handleFileChange} accept=".js,.zip" />
                    </div>
                    <div className="text-black">{result}</div>
                  </div>
                  <Button onClick={handleScanStart} disabled={!file || isScanning} className="bg-orange-500 text-white hover:bg-orange-600">
                    {isScanning ? 'Scanning...' : 'Start Scan'}
                  </Button>
                </DialogContent>
              </Dialog>
            </div>
          </div>
        </section>
      </main>
      <footer className="flex flex-col gap-2 sm:flex-row py-6 w-full shrink-0 items-center px-4 md:px-6 border-t border-orange-200">
        <p className="text-xs text-gray-500 dark:text-gray-400">© 2024 GitPatrol. All rights reserved.</p>
        <nav className="sm:ml-auto flex gap-4 sm:gap-6">
          <a className="text-xs hover:underline underline-offset-4 text-orange-500" href="#">
            Terms of Service
          </a>
          <a className="text-xs hover:underline underline-offset-4 text-orange-500" href="#">
            Privacy
          </a>
        </nav>
      </footer>
    </div>
  )
}