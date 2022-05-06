import { AccessCode } from 'src/app/accesscode/model/entities/accessCode.entity';
export declare class Queue {
    id: number;
    modificationCounter?: number;
    name?: string;
    logo?: string;
    currentNumber?: string;
    attentionTime: Date;
    minAttentionTime?: Date;
    active?: boolean;
    accessCodes: AccessCode[];
}
