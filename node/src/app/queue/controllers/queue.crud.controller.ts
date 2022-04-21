import { Controller, HttpStatus, Param, Res } from '@nestjs/common';
import { Crud, CrudRequest, Override, ParsedBody, ParsedRequest } from '@nestjsx/crud';
import { CrudType } from '@devon4node/common/serializer';
import { Queue } from '../model/entities/queue.entity';
import { QueueCrudService } from '../services/queue.crud.service';
import { ApiTags } from '@nestjs/swagger';
import { Response } from 'express';
import { plainToClass } from 'class-transformer';

@Crud({
  model: {
    type: Queue,
  },
})
@CrudType(Queue)
@Controller('queuemanagement/v1/queue')
@ApiTags('Queue')
export class QueueCrudController {
  constructor(public service: QueueCrudService) {}

  @Override('createOneBase')
  async create(@ParsedRequest() req: CrudRequest, @ParsedBody() dto: Queue, @Res() res: Response) {
    const resData = plainToClass(Queue, await this.service.createOne(req, dto));
    res.status(HttpStatus.OK).send(resData);
  }

  @Override('getOneBase')
  async get(@ParsedRequest() req: CrudRequest, @Res() res: Response) {
    try {
      const resData = plainToClass(Queue, await this.service.getOne(req));
      res.status(HttpStatus.OK).send(resData);
    } catch {
      res.status(HttpStatus.NOT_FOUND).send();
    }
  }

  @Override('deleteOneBase')
  async delete(@ParsedRequest() req: CrudRequest, @Param('id') id: number, @Res() res: Response) {
    try {
      await this.service.deleteOne(req);
      res.status(HttpStatus.OK).json(id);
    } catch {
      res.status(HttpStatus.NOT_FOUND).send();
    }
  }
}
