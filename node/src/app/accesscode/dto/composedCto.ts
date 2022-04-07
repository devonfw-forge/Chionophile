import { Queue } from '../../queue/model/entities/queue.entity';
import { Visitor } from 'src/app/visitor/model/entities/visitor.entity';
import { AccessCodeResponse } from './accessCodeResponse';

export class ComposedCTO {
  accessCode: AccessCodeResponse;
  queue: Queue;
  visitor: Visitor;
}
