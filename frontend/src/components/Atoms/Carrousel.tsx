import Image from "next/image";
type Props = {};
function Carrousel({}: Props) {
    return (
        <div
            className="hidden bg-gradient-to-br from-fuchsia-400
     via-fuchsia-900 to-purple-700  w-full h-full col-span-4 
     md:flex justify-center items-center flex-col gap-20 relative  rounded-xl container mx-auto"
        >
            <h2 className="text-neutral-100 text-6xl mx-20">
                Take Control Of Budget Now
            </h2>
            <Image
                className="rounded-3xl"
                priority
                src="/LoginPage.png"
                width={500}
                height={500}
                alt="logo"
            />
            <p className="absolute bottom-2 p-20 text-neutral-200">
                Lorem ipsum dolor sit amet consectetur, adipisicing elit.
                Officia, expedita. Neque magnam veniam sequi amet aut adipisci,
                incidunt mollitia autem dolores velit nihil a cumque omnis
                maiores, labore iure repellat.
            </p>
        </div>
    );
}
export default Carrousel;
