import styles from '@/styles/Vote.module.css'
import { useState } from 'react'


export default function Vote() {
    const [loading, setLoading] = useState<boolean|undefined>(undefined)
    async function vote(choice: string) {
        setLoading(true)
        await fetch('/api/vote', {
            method: "POST",
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({vote: choice})
        })
        setLoading(false)
    }
    return (
        <> 
            <h2 className={styles.h2}>Vote for your favorite animal</h2>
            <button className={styles.button} onClick={() => vote('Dogs')}>Dogs 🐶</button>
            <button className={styles.button} onClick={() => vote('Cats')}>Cats 😺</button>
            {
                loading ? 
                <>
                    <h3 className={styles.h3}>Loading...</h3>
                </> : <></>
            }
        </>
    )
}