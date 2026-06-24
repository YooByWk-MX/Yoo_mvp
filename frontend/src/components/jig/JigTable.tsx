'use client';

import { Jig, Dictionary } from '@/types/jig';

interface JigTableProps {
  jigs: Jig[];
  dict: Dictionary;
}

export default function JigTable({ jigs, dict }: JigTableProps) {
  return (
    <div className="w-full overflow-x-auto rounded-xl border border-gray-700 shadow-2xl bg-gray-800">
      <table className="w-full text-left border-collapse min-w-[600px] sm:min-w-full">
        <thead>
          <tr className="bg-gray-900 text-gray-400 text-xs sm:text-sm border-b border-gray-700">
            <th className="p-4 w-16 text-center">{dict.jig.tableNo}</th>
            <th className="p-4">{dict.jig.tableProv}</th>
            <th className="p-4 w-24 text-center">{dict.jig.tableQty}</th>
            <th className="p-4 w-32">{dict.jig.tableTablero}</th>
            <th className="p-4 w-32">{dict.jig.tableTypes}</th>
          </tr>
        </thead>
        <tbody className="divide-y divide-gray-700 text-sm">
          {jigs.map((jig) => (
            <tr key={jig.no} className="hover:bg-gray-700/30 transition text-gray-300">
              <td className="p-4 text-center font-bold text-blue-400">{jig.no}</td>
              <td className="p-4 font-sans whitespace-pre-wrap leading-relaxed break-all">{jig.prov}</td>
              <td className="p-4 text-center font-semibold bg-gray-900/20">{jig.cantidad}</td>
              <td className="p-4 text-gray-400">{jig.tablero || '-'}</td>
              <td className="p-4">
                <span className="inline-block bg-blue-950/50 text-blue-300 px-2 py-1 rounded text-xs border border-blue-900/50">
                  {jig.types || '-'}
                </span>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}