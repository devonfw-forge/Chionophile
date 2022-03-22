import { Exclude, Expose } from 'class-transformer';
import { IsOptional } from 'class-validator';

@Exclude()
export class VisitorDTO {
  @Expose()
  id: number;

  @Expose()
  @IsOptional()
  modificationCounter: number;

  @Expose()
  username: string;

  @Expose()
  name: string;

  @Expose()
  password: string;

  @Expose()
  phoneNumber: string;

  @Expose()
  acceptedCommercial: boolean;

  @Expose()
  acceptedTerms: boolean;

  @Expose()
  userType: boolean;
}
