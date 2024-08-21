export type Language = {
    id: number,
    name: string
}

export type Verdict = {
    input: string,
    output: string,
    answer: string,
    status: string,
    time: number,
    memory: number
}

export type Problem = {
    url: string,
    memory_limit: number,
    time_limit: number,
    title: string,
}