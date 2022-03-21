import { Controller } from '@nestjs/common';
import { Crud } from '@nestjsx/crud';
import { CrudType } from '@devon4node/common/serializer';
import { Queue } from '../model/entities/queue.entity';
import { QueueCrudService } from '../services/queue.crud.service';
import { ApiTags } from '@nestjs/swagger';

@Crud({
  model: {
    type: Queue,
  },
})
@CrudType(Queue)
@Controller('queue/queues')
@ApiTags('Queue')
export class QueueCrudController {
  constructor(public service: QueueCrudService) {}
}
