import { IsOptional } from 'class-validator';
import { Pageable } from '../../shared/interfaces/pageable';

export class Criteria {
  @IsOptional()
  active: boolean;
  pageable: Pageable;
}
