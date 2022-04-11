import { Exclude, Expose } from 'class-transformer';
import { IsOptional } from 'class-validator';
import { Pageable } from '../../shared/interfaces/pageable';

@Exclude()
export class Criteria {
  @Expose()
  @IsOptional()
  username: string;

  @Expose()
  @IsOptional()
  name: string;

  @Expose()
  @IsOptional()
  phoneNumber: string;

  @Expose()
  @IsOptional()
  acceptedCommercial: boolean;

  @Expose()
  @IsOptional()
  acceptedTerms: boolean;

  @Expose()
  @IsOptional()
  userType: boolean;

  @Expose()
  pageable: Pageable;
}
