import {client} from "@repo/db/client"
import type { Request, Response } from "express";

export async function postwebsiteDetails(req: Request, res:Response) {
    try {
      if(!req.body.url){
        res.status(411).json({})
        return;
      }

        const website = await client.website.create({
            data:{
                url:req.body.url,
                timeAdded: new Date()
            }
        })
  
      return res.status(201).json(website);
    } catch (error) {
      console.error("Error creating user:", error);
      return res.status(500).json({ message: "Internal server error" });
    }
  }
  export async function getwebsiteDetails(req:Request, res:Response) {
    try {
        const websiteId = req.params.websiteId;
        const website = await client.website.findUnique({
            where:{
                id: websiteId
            }
        })
        
  
      return res.status(201).json(website);
    } catch (error) {
      console.error("Error creating user:", error);
      return res.status(500).json({ message: "Internal server error" });
    }
  }
  
