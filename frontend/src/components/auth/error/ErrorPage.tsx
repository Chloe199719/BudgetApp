"use client";

import { useParams } from "next/navigation";

type Props = {};
function ErrorPage({}: Props) {
    const params = useParams<{ error: string }>();
    return <div>{params.error ? params.error : "Something went wrong"}</div>;
}
export default ErrorPage;
