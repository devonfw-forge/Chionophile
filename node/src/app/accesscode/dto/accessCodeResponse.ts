import { Exclude, Expose, Transform } from 'class-transformer';
@Exclude()
export class AccessCodeResponse {
  @Expose()
  modificationCounter: number;
  @Expose()
  @Transform(({ value }) => parseInt(value))
  id: number;
  @Expose()
  ticketNumber: string;
  @Expose()
  creationTime: Date;
  @Expose()
  @Transform(({ value }) => parseInt(value))
  visitorId: number;
  @Expose()
  @Transform(({ value }) => parseInt(value))
  queueId: number;
}
