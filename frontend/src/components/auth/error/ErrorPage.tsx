"use client";

import { useSearchParams } from "next/navigation";

type Props = {};
function ErrorPage({}: Props) {
    const params = useSearchParams();
    const error = params.get("error");
    return <div>{error ? error : "Something went wrong"}</div>;
}
export default ErrorPage;
