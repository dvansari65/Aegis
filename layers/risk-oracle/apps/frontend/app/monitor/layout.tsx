import Navbar from "@/components/navbar";

export default function MonitorLayout({ children }: { children: React.ReactNode }) {
  return (
    <div className="min-h-screen bg-skyglass-mist/40">
      <Navbar />
      <div className="mx-auto w-full max-w-7xl px-5 pb-16 pt-24 md:px-8">{children}</div>
    </div>
  );
}
