// app/docs/page.tsx
import { redirect } from 'next/navigation';

export default function DocsIndexPage() {
  redirect('/docs/protocol-overview'); // matches your first sidebar link
}