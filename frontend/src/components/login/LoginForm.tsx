"use client";
import { zodResolver } from "@hookform/resolvers/zod";
import axios from "axios";
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
const formSchema = z.object({
  email: z.string().email(),
  password: z.string().min(2),
});

type Props = {};
function LoginForm({}: Props) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      email: "",
      password: "",
    },
  });

  const dispatch = useDispatch();

  async function onsubmit(e: z.infer<typeof formSchema>) {
    const res = await axios.post(
      "http://localhost:5000/users/login/",
      {
        email: e.email,
        password: e.password,
      },
      { withCredentials: true }
    );
    const data = res.data as CurrentUserData;
    if (!data) return;
    dispatch(login({ ...data, isAuthenticated: true }));
  }
  return (
    <div className="w-full col-span-2  px-8 h-full flex items-center justify-center flex-col gap-8 max-w-xl">
      <h2 className="text-4xl text-pretty font-bold ">
        Sign in to your <span className="text-blue-500"> {APP_NAME} </span>{" "}
        Account
      </h2>
      <h3 className="text-start w-full text-lg">Hi, Welcome Back.👋 </h3>
      <div className="w-full flex flex-col">
        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onsubmit)}
            className="space-y-4 bg-blue-200 p-8 rounded-md"
          >
            <h3 className="text-center text-xl">Login</h3>
            <FormField
              control={form.control}
              name="email"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Email</FormLabel>
                  <FormControl>
                    <Input placeholder="Chloe" {...field} />
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
                    <Input type="password" placeholder="Password" {...field} />
                  </FormControl>

                  <FormMessage />
                </FormItem>
              )}
            />
            <div className="flex justify-end">
              <Link href="/forgot-password" className="text-blue-500">
                Forgot Password?
              </Link>
            </div>
            <Button className="w-full" type="submit">
              Login
            </Button>
            <div className="flex justify-center">
              <FormDescription>
                Don't have an account?{" "}
                <Link href="/register" className="text-blue-500">
                  Sign up
                </Link>
              </FormDescription>
            </div>
          </form>
        </Form>
      </div>
    </div>
  );
}
export default LoginForm;
