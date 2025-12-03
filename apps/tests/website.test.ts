import axios from "axios"
import {describe, expect, it} from "bun:test"

// const base_url = "http://localhost:3000/api/v1"
const base_url = "http://localhost:3002"


describe("website get creeated", ()=>{
  it("Website not created if url is not present", async()=>{
    try {
        await axios.post(`${base_url}/website/`, {
    
        })
        expect(false, "website created when it shouldn't")
    } catch (error) {
        
    }
  })
  it("Website created if url is present", async()=>{
        const res = await axios.post(`${base_url}/website`, {
            url:"amazon.com"
        })
      
        expect(res.data.id).not.toBeNull();
     
  })
})

 