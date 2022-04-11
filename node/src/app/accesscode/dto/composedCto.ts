import { Queue } from '../../queue/model/entities/queue.entity';
import { Visitor } from 'src/app/visitor/model/entities/visitor.entity';
import { AccessCodeResponse } from './accessCodeResponse';
import { Exclude, Expose } from 'class-transformer';

@Exclude()
export class ComposedCTO {
  @Expose()
  accessCode: AccessCodeResponse;
  @Expose()
  queue: Queue;
  @Expose()
  visitor: Visitor;
}
