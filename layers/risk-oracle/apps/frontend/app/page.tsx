import Architecture from "@/components/architecture";
import Benefits from "@/components/benefits";
import Flow from "@/components/flow";
import Footer from "@/components/footer";
import Hero from "@/components/hero";
import Navbar from "@/components/navbar";

export default function Home() {
  return (
    <main data-testid="landing-page" className="relative overflow-hidden">
      <Navbar />
      <Hero />
      <Architecture />
      <Flow />
      <Benefits />
      <Footer />
    </main>
  );
}
