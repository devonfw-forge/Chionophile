import { IsOptional } from 'class-validator';
import { Pageable } from '../../shared/interfaces/pageable';

export class Criteria {
  @IsOptional()
  visitorId: string;
  pageable: Pageable;
}
