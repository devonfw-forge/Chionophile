import { Pageable } from '../../shared/interfaces/pageable';
import { VisitorDTO } from './visitordto';

export class VisitorResponseDTO {
  content: VisitorDTO[];
  pageable: Pageable;
  totalElements: number;
}
