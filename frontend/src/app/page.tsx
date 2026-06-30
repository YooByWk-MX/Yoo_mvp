// src/app/page.tsx
"use client";

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { useAppStore, dict } from '@/store/useAppStore';
import { apiFetch } from '@/lib/api';

export default function LoginPage() {
  const router = useRouter();
  const { lang, setLang, setAuth } = useAppStore();
  const t = dict[lang];
  
  const [empno, setEmpno] = useState('');
  
  const handleLogin = async (e: React.FormEvent) => {
    e.preventDefault();
    try {
      const res = await apiFetch('/users/login', {
        method: 'POST',
        body: JSON.stringify({ empno, password: empno }) // MVP: 비밀번호는 사번과 동일
      });
      setAuth(res.token, res.role, empno);
      if (res.role === 'ADMIN') router.push('/admin');
      else router.push('/worker');
    } catch (err: any) {
      alert(err.message);
    }
  };

  return (
    <div className="min-h-screen bg-[#e8eef7] flex items-center justify-center p-4">
      <div className="bg-white rounded-2xl shadow-sm p-8 w-full max-w-md border border-slate-200">
        <div className="flex justify-end mb-4 bg-slate-100 rounded-lg p-1 w-fit ml-auto">
          <button onClick={() => setLang('ko')} className={`px-3 py-1 rounded-md text-sm font-bold ${lang === 'ko' ? 'bg-white shadow text-slate-800' : 'text-slate-500'}`}>KO</button>
          <button onClick={() => setLang('es')} className={`px-3 py-1 rounded-md text-sm font-bold ${lang === 'es' ? 'bg-white shadow text-slate-800' : 'text-slate-500'}`}>ES</button>
        </div>
        
        <h1 className="text-3xl font-black text-slate-800 mb-8 tracking-tight">YURA SYSTEM</h1>
        
        <form onSubmit={handleLogin} className="flex flex-col gap-4">
          <input 
            type="text" 
            placeholder={t.empno}
            value={empno}
            onChange={(e) => setEmpno(e.target.value)}
            className="p-4 bg-slate-50 border border-slate-200 rounded-xl focus:outline-none focus:border-blue-500 font-bold"
            required
          />
          <button type="submit" className="bg-slate-800 text-white p-4 rounded-xl font-bold text-lg hover:bg-slate-700 transition">
            {t.login}
          </button>
        </form>
      </div>
    </div>
  );
}