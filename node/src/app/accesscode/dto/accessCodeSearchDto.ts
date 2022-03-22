import { ComposedCTO } from './composedCto';
import { Pageable } from '../../shared/interfaces/pageable';

export class AccessCodeSearchDTO {
  content: ComposedCTO[];
  pageable: Pageable;
  totalElements: number;
}