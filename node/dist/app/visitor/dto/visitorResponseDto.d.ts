import { Pageable } from '../../shared/interfaces/pageable';
import { VisitorDTO } from './visitordto';
export declare class VisitorResponseDTO {
    content: VisitorDTO[];
    pageable: Pageable;
    totalElements: number;
}
