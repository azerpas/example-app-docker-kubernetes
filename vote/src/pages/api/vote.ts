// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import { randomBytes } from 'crypto'
import type { NextApiRequest, NextApiResponse } from 'next'
import {createClient} from '@redis/client'

type Data = {
  name: string
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
    if (req.method === 'POST') {
        const voterId = randomBytes(64).toString('base64')
        let vote = 'Dogs'
        try {
            vote = JSON.parse(req.body).vote
        } catch (error) {
            console.error(error)
            console.info('Missing param, voting for dogs then')
        }
        const client = createClient({
            url: "redis://redis:6379"
        })
        await client.connect()
        console.info(`Voting for ${vote}`)
        await client.rPush('votes', JSON.stringify({'voter_id': voterId, 'vote': vote}))
        res.setHeader('Set-Cookie', `voter_id=${voterId}; SameSite=Lax`)
        res.status(200).json({
            name: 'VoteCounted'
        })
    } else {
        res.status(400).json({
            name: 'MethodNotSupported'
        })
    }
}
