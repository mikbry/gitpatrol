import React, { useState, useCallback } from 'react';
import { Check, FileUp, SearchCode, Shield } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogFooter,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';

interface ScanDialogProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  report: string;
  onScan: (text: string | undefined) => Promise<void>;
}

export default function ScanDialog({ isOpen, report, onOpenChange, onScan }: ScanDialogProps) {
  const [file, setFile] = useState<File | null>(null);
  const [isScanning, setIsScanning] = useState(false);
  const [scanComplete, setScanComplete] = useState(false);
  const [dragActive, setDragActive] = useState(false);

  const handleDrag = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.type === 'dragenter' || e.type === 'dragover') {
      setDragActive(true);
    } else if (e.type === 'dragleave') {
      setDragActive(false);
    }
  }, []);

  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    e.stopPropagation();
    setDragActive(false);
    if (e.dataTransfer.files && e.dataTransfer.files[0]) {
      setFile(e.dataTransfer.files[0]);
      setScanComplete(false);
    }
  }, []);

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    e.preventDefault();
    if (e.target.files && e.target.files[0]) {
      setFile(e.target.files[0]);
      setScanComplete(false);
    }
  };

  const handleScanStart = async () => {
    if (file) {
      setIsScanning(true);
      // Simulating a scan process
      /* setTimeout(() => {
        setIsScanning(false);
        setScanComplete(true);
      }, 3000); */
      const text = await file.text();
      await onScan(text);
      setIsScanning(false);
      setScanComplete(true);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2 text-2xl font-bold text-orange-500 dark:text-orange-400">
            <Shield className="h-6 w-6" />
            GitPatrol malware scanner
          </DialogTitle>
        </DialogHeader>
        <div>
          <div className="grid w-full max-w-sm items-center gap-1.5">
            <div
              className={`flex h-32 w-full cursor-pointer flex-col items-center justify-center rounded-lg border-2 border-dashed transition-colors ${
                dragActive
                  ? 'border-orange-500 bg-orange-50 dark:bg-orange-900'
                  : 'border-gray-300 dark:border-gray-600'
              }`}
              onDragEnter={handleDrag}
              onDragLeave={handleDrag}
              onDragOver={handleDrag}
              onDrop={handleDrop}
            >
              <Input
                id="file"
                type="file"
                className="hidden"
                onChange={handleChange}
                accept=".js,.zip"
              />
              <label
                htmlFor="file"
                className="flex h-full w-full flex-col items-center justify-center"
              >
                <FileUp className="mb-2 h-8 w-8 text-gray-500 dark:text-gray-400" />
                <p className="text-sm text-gray-500 dark:text-gray-400">
                  {file ? file.name : 'Drag and drop or click to upload a suspicious file'}
                </p>
              </label>
            </div>
          </div>
          {scanComplete && (
            <div className="flex items-center gap-2 text-green-600 dark:text-green-400">
              <Check className="h-5 w-5" />
              <span>Scan completed successfully!</span>
            </div>
          )}
          {!scanComplete && !report && (
            <div className="m-2 text-sm font-medium text-gray-700 dark:text-gray-300">
              This a demo of GitPatrol, the scan is running on your machine, nothing is send to any
              server. It is a secure and sandboxed operation using WASM and Rust. If you think
              results are not correct, add an issue on our Github repository. Thanks !
            </div>
          )}
        </div>
        <DialogFooter className="sm:justify-start">
          <Button
            type="button"
            onClick={handleScanStart}
            disabled={!file || isScanning}
            className="w-full bg-orange-500 text-white transition-colors duration-200 hover:bg-orange-600 disabled:cursor-not-allowed disabled:opacity-50"
          >
            {isScanning ? (
              <>
                <svg
                  className="-ml-1 mr-3 h-5 w-5 animate-spin text-white"
                  xmlns="http://www.w3.org/2000/svg"
                  fill="none"
                  viewBox="0 0 24 24"
                >
                  <circle
                    className="opacity-25"
                    cx="12"
                    cy="12"
                    r="10"
                    stroke="currentColor"
                    strokeWidth="4"
                  />
                  <path
                    className="opacity-75"
                    fill="currentColor"
                    d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                  />
                </svg>
                Scanning...
              </>
            ) : (
              <>
                <SearchCode className="mr-2 h-5 w-5" />
                Start Scan
              </>
            )}
          </Button>
        </DialogFooter>
        {scanComplete && (
          <div className="mt-4 rounded-md bg-green-100 p-3 dark:bg-green-800">
            <p className="flex items-center text-sm text-green-800 dark:text-green-200">
              <Check className="mr-2 h-5 w-5" />
              Scan completed for {file?.name}. <br />
              {report}
            </p>
          </div>
        )}
      </DialogContent>
    </Dialog>
  );
}
