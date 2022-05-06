export interface Visitor {
    id?: number;
    username: string;
    name: string;
    password: string;
    phoneNumber: string;
    acceptedCommercial: boolean;
    acceptedTerms: boolean;
    userType: boolean;
}
export interface VisitorArray {
    content: Visitor[];
}
export interface FilterVisitor {
    pageable: Pageable;
    username?: string;
    password?: string;
}
export interface Sort {
    property: string;
    direction: string;
}
export interface Role {
    name: string;
    permission: number;
}
export interface Pageable {
    pageable: {
        pageNumber: number;
        pageSize: number;
        sort: Array<String>;
    };
}
export interface FilterAccessCode {
    pageable: Pageable;
    visitorId?: Number;
    endTime?: string;
}
export interface FilterQueue {
    pageable: Pageable;
    active: boolean;
}
export interface AccessCode {
    id?: number;
    ticketNumber: string;
    creationTime: string;
    startTime?: string;
    endTime?: string;
    visitorId: number;
    queueId: number;
    content: any;
}
export interface Queue {
    id?: number;
    name: string;
    logo: string;
    currentNumber: string;
    attentionTime: string;
    minAttentionTime: string;
    active: boolean;
    customers: number;
    content: any;
}
export interface QueueArray {
    content: Queue[];
}
export interface AccessCodeArray {
    content: [
        {
            accessCode: AccessCode;
        }
    ];
}
