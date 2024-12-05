'use client'

import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/tauri'
import { readText } from '@tauri-apps/api/clipboard';


export default function Greet() {
  const [greeting, setGreeting] = useState('');
    
    useEffect(() => {
        const handleFocus = () => {
        invoke<string>('greet', { name: 'Next.js' })
      .then(result => setGreeting(result))
      .catch(console.error)
          // Code to run when the window is brought into focus
          console.log('Window is in focus');
        };
    
        window.addEventListener('focus', handleFocus);
    
        // Clean up the event listener when the component unmounts
        return () => {
          window.removeEventListener('focus', handleFocus);
        };
      }, []);

    
    useEffect(() =>
    {
        invoke<string>('greet', { name: 'Next.js' })
      .then(result => setGreeting(result))
      .catch(console.error)
    }, [])


  // Necessary because we will have to use Greet as a component later.
  return <div>{greeting}</div>;
}