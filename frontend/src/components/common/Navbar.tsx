'use client';

import { Lang } from '@/types/jig';

interface NavbarProps {
  currentLang: Lang;
  onLangChange: (lang: Lang) => void;
}

export default function Navbar({ currentLang, onLangChange }: NavbarProps) {
  return (
    <header className="w-full bg-gray-800 border-b border-gray-700 px-6 py-4 flex justify-between items-center shadow-md">
      <div className="flex items-center space-x-3">
        <span className="text-xl font-black tracking-wider text-blue-500 font-sans">YURA HARNESS MEXICO </span>
        <span className="hidden sm:inline text-xs text-gray-400 bg-gray-900 px-2 py-1 rounded">INTERNAL ONLY</span>
      </div>
      <div className="flex bg-gray-900 rounded p-1 border border-gray-700">
        <button 
          onClick={() => onLangChange('ko')} 
          className={`px-3 py-1 text-xs font-bold rounded transition ${currentLang === 'ko' ? 'bg-blue-600 text-white' : 'text-gray-400 hover:text-gray-200'}`}
        >
          KO
        </button>
        <button 
          onClick={() => onLangChange('es')} 
          className={`px-3 py-1 text-xs font-bold rounded transition ${currentLang === 'es' ? 'bg-blue-600 text-white' : 'text-gray-400 hover:text-gray-200'}`}
        >
          ES
        </button>
      </div>
    </header>
  );
}