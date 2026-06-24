'use client';

import { useState, useEffect, Suspense } from 'react';
import { useSearchParams } from 'next/navigation';
import { Jig, CreateJigDto, Lang } from '@/types/jig';
import { getDictionary } from '@/i18n/dictionaries';
import { jigApi } from '@/services/api';
import Navbar from '@/components/common/Navbar';
import JigForm from '@/components/jig/JigForm';
import JigTable from '@/components/jig/JigTable';

function JigsContent() {
  const searchParams = useSearchParams();
  const initialLang = (searchParams.get('lang') as Lang);
  const [lang, setLang] = useState<Lang>(initialLang === 'es' || initialLang === 'ko' ? initialLang : 'ko');
  const [jigs, setJigs] = useState<Jig[]>([]);
  const [loading, setLoading] = useState(true);
  const dict = getDictionary(lang);

  const loadJigs = async () => {
    try {
      const data = await jigApi.getAll();
      setJigs(data);
    } catch (e) {
      console.error('API Error:', e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => { loadJigs(); }, []);

  const handleCreateJig = async (dto: CreateJigDto) => {
    await jigApi.create(dto);
    await loadJigs(); 
  };

  return (
    <div className="min-h-screen bg-gray-900 text-gray-100 flex flex-col font-mono">
      <Navbar currentLang={lang} onLangChange={setLang} />
      
      <main className="flex-1 p-4 sm:p-8 max-w-6xl w-full mx-auto space-y-8">
        <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center border-b border-gray-800 pb-4 gap-2">
          <h1 className="text-2xl sm:text-3xl font-black text-blue-400 tracking-tight">{dict.jig.title}</h1>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 items-start">
          <div className="lg:col-span-1">
            <JigForm dict={dict} onSubmit={handleCreateJig} />
          </div>
          <div className="lg:col-span-2">
            {loading ? (
              <div className="text-center py-12 text-sm text-gray-500 animate-pulse">{dict.jig.loading}</div>
            ) : (
              <JigTable jigs={jigs} dict={dict} />
            )}
          </div>
        </div>
      </main>
    </div>
  );
}

export default function JigsPage() {
  return (
    <Suspense fallback={<div className="min-h-screen bg-gray-900 text-gray-100 flex items-center justify-center font-mono">Cargando...</div>}>
      <JigsContent />
    </Suspense>
  );
}