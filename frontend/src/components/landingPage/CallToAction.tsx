import { APP_NAME } from "@/lib/constants";
import Image from "next/image";
import Link from "next/link";

type Props = {};
function CallToAction({}: Props) {
  return (
    <section className="py-10 flex-1 w-full flex bg-gradient-to-bl from-emerald-400 to-blue-400/95 gap-4  items-center relative justify-center mx-auto ">
      <div className="w-full  flex flex-1 items-center justify-center">
        <div className=" p-20  flex -translate-y-10 space-x-64 z-10 max-w-7xl ">
          <div className=" flex-1 flex flex-col gap-3">
            <h2 className="text-gray-200 text-7xl">
              Take Control of Your Finances Today!
            </h2>
            <h3 className="text-gray-300 text-4xl ml-2">
              Discover {APP_NAME}, Your Personal Budgeting Assistant!
            </h3>
            <ul className="font-bold text-gray-800 text-2xl list-disc mt-5 ml-10">
              <li>Stay on Track: Effortlessly monitor your spending.</li>
              <li>Save Smartly: Set goals and watch your savings grow.</li>
              <li>
                Gain Insights: Understand your habits with easy-to-read charts.
              </li>
              <li>Secure & Private: Your data stays safe with us.</li>
            </ul>
            <div className="flex justify-end mt-5">
              <Link
                className="bg-gradient-to-bl from-emerald-400 to-blue-400/95 opacity-90 border-2 border-black p-2 rounded-lg  hover:from-emerald-200 hover:to-blue-400/55 active:-translate-y-1"
                href={`/register`}
              >
                🌟 Get Started Now! 🌟
              </Link>
            </div>
          </div>
        </div>

        <div className="p-4 -translate-y-10 -translate-x-16 z-10">
          <Image
            src={`/picture2.png`}
            alt="Call To Action"
            width={500}
            height={1000}
          />
        </div>
      </div>
      <Image
        className="absolute w-full h-full object-cover opacity-30 contrast-125 grayscale"
        src={`/background1.jpg`}
        alt="Call To Action"
        width={1920}
        height={1080}
      />
    </section>
  );
}
export default CallToAction;
