// src/app/admin/page.tsx
"use client";

import { useEffect, useState } from "react";
import { QRCodeSVG } from "qrcode.react";
import { apiFetch } from "@/lib/api";
import { useAppStore, dict } from "@/store/useAppStore";
import { LogOut } from "lucide-react";
import { useRouter } from "next/navigation";

export default function AdminPage() {
  const router = useRouter();
  const { lang, logout } = useAppStore();
  const t = dict[lang];
  const [qrToken, setQrToken] = useState<string | null>(null);

  const fetchQR = async () => {
    try {
      const res = await apiFetch("/attendance/qr-generate");
      setQrToken(res.qr_data);
    } catch (err) {
      console.error(err);
    }
  };

  useEffect(() => {
    fetchQR();
    const interval = setInterval(fetchQR, 20000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="min-h-screen bg-slate-900 flex flex-col items-center justify-center p-4">
      <div className="absolute top-4 right-4 text-white">
        <button onClick={() => { logout(); router.push('/'); }}><LogOut /></button>
      </div>
      <h1 className="text-4xl font-black text-white mb-8 tracking-wider">{t.admin_qr}</h1>
      <div className="bg-white p-6 rounded-3xl shadow-2xl">
        {qrToken ? (
          <QRCodeSVG value={qrToken} size={320} />
        ) : (
          <div className="w-[320px] h-[320px] flex items-center justify-center font-bold text-slate-400">
            {t.loading}
          </div>
        )}
      </div>
    </div>
  );
}