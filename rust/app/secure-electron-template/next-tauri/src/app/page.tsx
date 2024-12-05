import Image from "next/image";
import { useEffect } from "react";
import Greet from "./greet";


async function getData() {
  try
  {
    const response = await fetch('https://www.yabla.com')   
    const contentType = response.headers.get('Content-Type');

      if (contentType && contentType.includes('application/json')) {
        return response.json();
      } else {
        return response.text();
      }
    }
  catch (err)
  {
    console.log(err)
    return ""
    // throw new Error('Failed to fetch data' + err)
  }
}


export default async function Home() {

  // const data = await getData();
  // console.log("DATA", data);
  return (
    <main className="flex min-h-screen flex-col items-center justify-between p-24">
      <Greet />
    </main>
  );
}
