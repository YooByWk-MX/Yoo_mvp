'use client';

import { useState } from 'react';
import Link from 'next/link';
import { Lang } from '@/types/jig';
import { getDictionary } from '@/i18n/dictionaries';
import Navbar from '@/components/common/Navbar';

export default function PortalPage() {
  const [lang, setLang] = useState<Lang>('ko');
  const dict = getDictionary(lang);

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col font-mono">
      <Navbar currentLang={lang} onLangChange={setLang} />
      
      <main className="flex-1 flex flex-col justify-center items-center p-4 text-center">
        <div className="max-w-md w-full bg-gray-800 border border-gray-700 p-6 sm:p-8 rounded-2xl shadow-2xl">
          <div className="w-16 h-16 bg-blue-600/10 border border-blue-500 rounded-2xl flex items-center justify-center mx-auto mb-6">
            <svg className="w-8 h-8 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 114 0v2m0 0h4m0 0V5a2 2 0 114 0v2" /></svg>
          </div>
          <h1 className="text-2xl sm:text-3xl font-extrabold mb-8 text-gray-100 tracking-tight">{dict.main.title}</h1>
          
          <Link href={`/jigs?lang=${lang}`} className="block w-full bg-blue-600 hover:bg-blue-500 text-white font-bold p-4 rounded-xl shadow-lg transition active:scale-[0.98] text-sm sm:text-base">
            {dict.main.portalBtn}
          </Link>
        </div>
      </main>
    </div>
  );
}