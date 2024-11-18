import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Input } from '@/components/ui/input';

export default function Page() {
  const [file, setFile] = useState<File | null>(null);
  const [result, setResult] = useState<string>('');

  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setFile(e.target.files[0]);
    }
  };

  const handleScan = async () => {
    if (!file) return;

    try {
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
  };

  return (
    <main className="container mx-auto p-4">
      <Card>
        <CardHeader>
          <CardTitle>Repository Scanner</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-4">
            <Input type="file" accept=".js,.zip" onChange={handleFileChange} />
            <Button onClick={handleScan} disabled={!file}>
              Scan File
            </Button>
            {result && <div className="mt-4 p-4 border rounded">{result}</div>}
          </div>
        </CardContent>
      </Card>
    </main>
  );
}
