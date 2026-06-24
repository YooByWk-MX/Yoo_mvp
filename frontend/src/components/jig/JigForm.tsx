'use client';

import { useState, FormEvent, ChangeEvent } from 'react';
import { CreateJigDto, Dictionary } from '@/types/jig';

export default function JigForm({ dict, onSubmit }: { dict: Dictionary, onSubmit: (d: CreateJigDto) => Promise<void> }) {
  const [form, setForm] = useState<CreateJigDto>({
    prov: 'no.',
    cantidad: 1,
    tablero: '',
    color: '',
    mf: 'Male',
    types: 'Ensamblaje',
    pin: '',
    note: ''
  });

  const handleFormSubmit = async (e: FormEvent) => {
    e.preventDefault();
    await onSubmit(form);
  };

  // 모든 input/textarea의 value에 적용할 안전한 변환 함수
  const val = (v: string | null | undefined): string => v ?? '';

  return (
    <form onSubmit={handleFormSubmit} className="bg-gray-800 p-6 rounded-xl grid grid-cols-2 gap-4 text-sm">
      <textarea 
        className="col-span-2 bg-gray-900 p-2 rounded text-white h-20" 
        value={val(form.prov)} 
        onChange={(e: ChangeEvent<HTMLTextAreaElement>) => setForm({...form, prov: e.target.value})} 
        placeholder="Prov" 
      />
      <input 
        type="number" 
        className="bg-gray-900 p-2 rounded text-white" 
        value={form.cantidad} 
        onChange={(e: ChangeEvent<HTMLInputElement>) => setForm({...form, cantidad: Number(e.target.value)})} 
      />
      <input 
        type="text" 
        className="bg-gray-900 p-2 rounded text-white" 
        value={val(form.tablero)} 
        onChange={(e: ChangeEvent<HTMLInputElement>) => setForm({...form, tablero: e.target.value})} 
        placeholder="Tablero" 
      />
      <input 
        type="text" 
        className="bg-gray-900 p-2 rounded text-white" 
        value={val(form.color)} 
        onChange={(e: ChangeEvent<HTMLInputElement>) => setForm({...form, color: e.target.value})} 
        placeholder="Color (A/B)" 
      />
      <select 
        className="bg-gray-900 p-2 rounded text-white" 
        value={form.mf} 
        onChange={(e: ChangeEvent<HTMLSelectElement>) => setForm({...form, mf: e.target.value})}
      >
        <option value="Male">Male</option>
        <option value="Female">Female</option>
      </select>
      <select 
        className="bg-gray-900 p-2 rounded text-white" 
        value={form.types} 
        onChange={(e: ChangeEvent<HTMLSelectElement>) => setForm({...form, types: e.target.value})}
      >
        <option value="Ensamblaje">Ensamblaje</option>
        <option value="Prueba">Prueba</option>
      </select>
      <input 
        type="text" 
        className="bg-gray-900 p-2 rounded text-white" 
        value={val(form.pin)} 
        onChange={(e: ChangeEvent<HTMLInputElement>) => setForm({...form, pin: e.target.value})} 
        placeholder="Pin" 
      />
      <input 
        type="text" 
        className="bg-gray-900 p-2 rounded text-white" 
        value={val(form.note)} 
        onChange={(e: ChangeEvent<HTMLInputElement>) => setForm({...form, note: e.target.value})} 
        placeholder="Note" 
      />
      <button type="submit" className="col-span-2 bg-blue-600 p-3 rounded font-bold text-white hover:bg-blue-700 transition">
        등록 (Registrar)
      </button>
    </form>
  );
}