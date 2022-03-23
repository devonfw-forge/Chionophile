import { Exclude, Expose } from 'class-transformer';
@Exclude()
export class AccessCodeResponse {
  @Expose()
  modificationCounter: number;
  @Expose()
  id: number;
  @Expose()
  ticketNumber: string;
  @Expose()
  creationTime: Date;
  @Expose()
  visitorId: number;
  @Expose()
  queueId: number;
}
