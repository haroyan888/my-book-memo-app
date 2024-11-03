import {
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import type { MetaFunction } from "@remix-run/node";
import "./tailwind.css";
import 'bootstrap/dist/css/bootstrap.min.css';
import {useEffect} from "react";

import {ORIGIN} from "~/consts";
import isLoggedIn from "~/utility/login/check-login";

export const meta: MetaFunction = () => {
    return [
        { title: "読書メモアプリ" },
        { name: "description", content: "Welcome to Remix!" },
    ];
};

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body>
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
    useEffect(() => {
        const authURLList = [
            ORIGIN + '/',
            ORIGIN + "/login",
            ORIGIN + "/create-account"
        ];
        const URL = document.URL;
        if(!authURLList.includes(URL)) (async () => {
            if (!await isLoggedIn()) document.location.href = "/login";
        })();
    }, []);
    return <Outlet />;
}
