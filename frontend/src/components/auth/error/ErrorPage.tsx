"use client";

import { useSearchParams } from "next/navigation";

type Props = {};
function ErrorPage({}: Props) {
    const params = useSearchParams();
    const error = params.get("error");
    return (
        <div className="text-amber-600 text-lg md:text-2xl">
            {error ? error : "Something went wrong"}
        </div>
    );
}
export default ErrorPage;
