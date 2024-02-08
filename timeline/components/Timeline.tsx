import { createClient } from '@/utils/supabase/server'
import { cookies } from 'next/headers'


export default async function Timeline() {
    const cookieStore = cookies()
    const supabase = createClient(cookieStore)


    const { data, error } = await supabase.from('timeline').select('*')

    if (error) {
        console.log(error)
        return <div>error</div>
    }
    console.log("dsata; " + data)
    return <pre>JSON: {JSON.stringify(data, null, 2)}</pre>


}