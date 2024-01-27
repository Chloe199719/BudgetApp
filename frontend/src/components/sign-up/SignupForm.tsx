"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import { useState } from "react";
import { CurrentUserData } from "../../app/layout";
import { useDispatch } from "@/lib/redux/store";
import { login } from "@/lib/redux/slices/auth";
import { z } from "zod";
import { useForm } from "react-hook-form";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormLabel,
    FormMessage,
} from "../ui/form";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { APP_NAME } from "@/lib/constants";
import Link from "next/link";
import { useRouter } from "next/navigation";
import axiosInstance from "@/lib/api/axios";
import { registerUserApi } from "@/lib/api/auth/register";
import { useToast } from "../ui/use-toast";
import { useMutation } from "react-query";

const formSchema = z
    .object({
        email: z.string().email(),
        password: z.string().min(2),
        confirmPassword: z.string().min(2),
        unique_name: z.string().min(2),
        display_name: z.string().min(2),
    })
    .refine((data) => data.password === data.confirmPassword, {
        message: "Passwords don't match",
        path: ["confirmPassword"],
    })
    .refine((data) => data.password === data.confirmPassword, {
        message: "Passwords don't match",
        path: ["password"],
    });

export type SignUpFormData = z.infer<typeof formSchema>;

function SignUpForm() {
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            email: "",
            password: "",
            confirmPassword: "",
            unique_name: "",
            display_name: "",
        },
    });
    const { toast } = useToast();
    const handleRegistration = useMutation(registerUserApi, {
        onSuccess: (data) => {
            toast({
                title: "Success",
                description: data.message,
            });
        },
        onError: (error: any) => {
            toast({
                title: "Error",
                description: error.error,
            });
        },
    });

    function onsubmit(e: z.infer<typeof formSchema>) {
        handleRegistration.mutate(e);
    }
    return (
        <div className="w-full col-span-2  px-8 h-full flex items-center justify-center flex-col gap-8 max-w-xl">
            <h2 className="text-4xl text-pretty font-bold ">
                Create Your <span className="text-blue-500"> {APP_NAME} </span>{" "}
                Account
            </h2>
            <h3 className="text-start w-full text-lg"> Welcome.👋 </h3>
            <div className="w-full flex flex-col">
                <Form {...form}>
                    <form
                        onSubmit={form.handleSubmit(onsubmit)}
                        className="space-y-4 bg-blue-300 p-8 rounded-md dark:bg-gray-800"
                    >
                        <h3 className="text-center text-xl">Login</h3>
                        <FormField
                            control={form.control}
                            name="email"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Email</FormLabel>
                                    <FormControl>
                                        <Input
                                            id="email"
                                            placeholder="chloe@example.com"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="unique_name"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>UserName</FormLabel>
                                    <FormControl>
                                        <Input
                                            type="text"
                                            id="unique_name"
                                            placeholder="chloeiscute"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="display_name"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Display Name</FormLabel>
                                    <FormControl>
                                        <Input
                                            type="text"
                                            id="display_name"
                                            placeholder="Chloe Pratas"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="password"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Password</FormLabel>
                                    <FormControl>
                                        <Input
                                            id="password"
                                            type="password"
                                            placeholder="Password"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />
                        <FormField
                            control={form.control}
                            name="confirmPassword"
                            render={({ field }) => (
                                <FormItem>
                                    <FormLabel>Confirm Password</FormLabel>
                                    <FormControl>
                                        <Input
                                            id="Confirm_Password"
                                            type="password"
                                            placeholder="Confirm Password"
                                            {...field}
                                        />
                                    </FormControl>

                                    <FormMessage />
                                </FormItem>
                            )}
                        />

                        <Button
                            disabled={handleRegistration.isLoading}
                            className={`w-full disabled:bg-gray-500 disabled:cursor-not-allowed disabled:dark:bg-gray-800`}
                            type="submit"
                        >
                            Register
                        </Button>
                        <div className="flex justify-center">
                            <FormDescription>
                                Already have an account?{" "}
                                <Link
                                    href="/auth/login"
                                    className="text-blue-500"
                                >
                                    Login
                                </Link>
                            </FormDescription>
                        </div>
                    </form>
                </Form>
            </div>
        </div>
    );
}
export default SignUpForm;
